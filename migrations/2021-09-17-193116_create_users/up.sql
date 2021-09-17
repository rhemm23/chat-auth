CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(20) NOT NULL,
  pass_hash VARCHAR(200) NOT NULL,
  pass_salt VARCHAR(200) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_users_username ON users(username);
