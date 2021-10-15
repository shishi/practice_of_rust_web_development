-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  unique_id VARCHAR NOT NULL,
  UNIQUE (email),
  UNIQUE (username)
);
INSERT INTO users (username, email, password, unique_id)
VALUES (
    'placeholder',
    'placeholder email',
    'placeholder password',
    'placeholder unique id'
  ),
  (
    'maxwell',
    'test@gmail.com',
    '$2b$12$jlfLwu4AHjrvTpZrB311Y.W0JulQ71WVy2g771xl50e5nS1UfqwQ.',
    '543b7aa8-e563-43e0-8f62-55211960a604'
  );
ALTER TABLE to_do
ADD user_id integer default 1 CONSTRAINT user_id REFERENCES users NOT NULL;
