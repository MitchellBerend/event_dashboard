package utils

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"log"
)


func DetectEventSchema(eventData map[string]interface{}) string {
	// Normalize event data to JSON string
	eventJSON, err := json.Marshal(eventData)
	if err != nil {
		log.Printf("Failed to marshal event data: %v", err)
		return ""
	}

	// Create a unique identifier for the schema
	hash := sha256.New()
	hash.Write(eventJSON)
	return hex.EncodeToString(hash.Sum(nil))
}
