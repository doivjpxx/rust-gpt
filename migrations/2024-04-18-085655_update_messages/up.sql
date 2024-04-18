-- Your SQL goes here
CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  given_message TEXT NOT NULL,
  message TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
