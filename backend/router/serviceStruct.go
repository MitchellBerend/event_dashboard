package router

import (
	"backend/utils"
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"

	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

type EventProcessor struct {
	mongoClient      *mongo.Client
	mongoDB          *mongo.Database
	eventsCollection *mongo.Collection
	dbConn           *sql.DB
}

func NewEventProcessor(mongoClient *mongo.Client, mongoDB *mongo.Database, eventsCollection *mongo.Collection, dbConn *sql.DB) *EventProcessor {
	return &EventProcessor{
		mongoClient:      mongoClient,
		mongoDB:          mongoDB,
		eventsCollection: eventsCollection,
		dbConn:           dbConn,
	}
}

func (ep *EventProcessor) HandleEvent(w http.ResponseWriter, r *http.Request) {
	var eventData map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&eventData); err != nil {
		log.Printf("Problem found: %e\n", err)
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	// Normalize and detect schema
	eventSchema := utils.DetectEventSchema(eventData)

	// Store event data in MongoDB
	if err := ep.storeEventData(eventSchema, eventData); err != nil {
		log.Printf("Problem found: %e\n", err)
		http.Error(w, "Failed to store event data", http.StatusInternalServerError)
		return
	}

	// Store event reference in PostgreSQL
	if err := ep.storeEventReference(eventSchema); err != nil {
		log.Printf("Problem found: %e\n", err)
		http.Error(w, "Failed to store event reference", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (ep *EventProcessor) storeEventData(schemaID string, eventData map[string]interface{}) error {
	_, err := ep.eventsCollection.InsertOne(context.TODO(), bson.M{
		"schemaId": schemaID,
		"metadata": eventData,
	})

	if mongo.IsDuplicateKeyError(err) {
		return nil
	}
	if err != nil {
		log.Printf("Failed to store event data: %v", err)
		return err
	}

	return err
}

func (ep *EventProcessor) GetEventReferences(w http.ResponseWriter, _ *http.Request) {
	rows, err := ep.dbConn.Query("SELECT reference, createdAt FROM event ORDER BY id DESC LIMIT 500")
	if err != nil {
		http.Error(w, "Failed to retrieve event references", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var references []utils.EventReference
	for rows.Next() {
		var ref utils.EventReference
		if err := rows.Scan(&ref.Reference, &ref.CreatedAt); err != nil {
			http.Error(w, "Failed to scan row", http.StatusInternalServerError)
			return
		}
		references = append(references, ref)
	}

	if err := rows.Err(); err != nil {
		http.Error(w, "Row iteration error", http.StatusInternalServerError)
		return
	}

	// Return the list of references
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(references)
}

func (ep *EventProcessor) GetEventByID(w http.ResponseWriter, r *http.Request) {
	params := mux.Vars(r)
	schemaID, ok := params["id"]
	if !ok {
		http.Error(w, "ID parameter is required", http.StatusBadRequest)
		return
	}

	var event utils.Event
	err := ep.eventsCollection.FindOne(context.TODO(), bson.M{"schemaId": schemaID}).Decode(&event)
	if err != nil {
		if err == mongo.ErrNoDocuments {
			http.Error(w, "Event not found", http.StatusNotFound)
		} else {
			http.Error(w, "Failed to retrieve event", http.StatusInternalServerError)
		}
		return
	}

	// Return the event data
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(event)
}

func (ep *EventProcessor) storeEventReference(schemaID string) error {
	query := `INSERT INTO event (reference) VALUES ($1) ON CONFLICT (reference) DO NOTHING`
	_, err := ep.dbConn.Exec(query, schemaID)
	return err
}
