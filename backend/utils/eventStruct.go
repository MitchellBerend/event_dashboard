package utils

import "time"

type Event struct {
	SchemaID string                 `json:"schemaId"`
	Metadata map[string]interface{} `json:"event"`
}

type EventReference struct {
	Reference string `json:"reference"`
	//CreatedAt string `json:"createdAt"`
	CreatedAt time.Time `json:"createdAt"`
}
