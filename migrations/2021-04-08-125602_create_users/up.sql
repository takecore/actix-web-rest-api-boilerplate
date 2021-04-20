-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  company_id INT NOT NULL references companies(id),
  name VARCHAR(50) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
  updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
)
