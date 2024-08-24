-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS event (
    id SERIAL PRIMARY KEY,
    reference CHAR(64) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE event;
-- +goose StatementEnd
