package main

import (
	"backend/router"
	"context"
	"database/sql"
	"embed"
	"log"
	"net/http"
	"os"

	"github.com/gorilla/mux"
	_ "github.com/lib/pq"
	"github.com/pressly/goose/v3"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

//go:embed migrations/*.sql
var embedMigrations embed.FS

func runMongoMigrations(client *mongo.Client) error {
	db := client.Database("events_db")

	// Example: Create an index on a collection
	eventsCollection := db.Collection("events")
	indexModel := mongo.IndexModel{
		Keys:    bson.D{{Key: "schemaId", Value: 1}}, // Create an index on the "schemaId" field
		Options: options.Index().SetUnique(true),
	}

	_, err := eventsCollection.Indexes().CreateOne(context.TODO(), indexModel)
	if err != nil {
		return err
	}

	// Additional migrations can go here

	log.Println("Migrations for Mongo completed successfully")
	return nil
}

func AutoMigrate(db *sql.DB) error {
	goose.SetBaseFS(embedMigrations)

	if err := goose.SetDialect("postgres"); err != nil {
		panic(err)
	}

	if err := goose.Up(db, "migrations"); err != nil {
		panic(err)
	}

	return nil
}

func main() {
	// MongoDB connection
	mongoURI := os.Getenv("MONGO_URI")
	clientOptions := options.Client().ApplyURI(mongoURI)
	mongoClient, err := mongo.Connect(context.TODO(), clientOptions)
	if err != nil {
		log.Fatalf("Failed to connect to MongoDB: %v", err)
	}
	defer mongoClient.Disconnect(context.TODO())
	if err := runMongoMigrations(mongoClient); err != nil {
		log.Fatalf("Mongo migration failed: %v", err)
	}

	mongoDB := mongoClient.Database("events_db")
	eventsCollection := mongoDB.Collection("events")

	// PostgreSQL connection
	postgresURI := os.Getenv("POSTGRES_URI")

	dbConn, err := sql.Open("postgres", postgresURI)
	if err != nil {
		log.Fatalf("Failed to connect to PostgreSQL: %v", err)
	}
	defer dbConn.Close()
	if err := AutoMigrate(dbConn); err != nil {
		log.Fatalf("Postgres migration failed: %v", err)
	}

	processor := router.NewEventProcessor(
		mongoClient,
		mongoDB,
		eventsCollection,
		dbConn,
	)

	r := mux.NewRouter()
	r.HandleFunc("/event", processor.HandleEvent).Methods("POST")
	r.HandleFunc("/events", processor.GetEventReferences).Methods("GET")
	r.HandleFunc("/event/{id:[a-fA-F0-9]{64}}", processor.GetEventByID).Methods("GET")

	http.Handle("/", enableCORS(r))
	log.Println("Listening on port 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}

// CORS middleware function
func enableCORS(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")

		// Handle preflight request
		if r.Method == "OPTIONS" {
			w.WriteHeader(http.StatusOK)
			return
		}

		next.ServeHTTP(w, r)
	})
}
