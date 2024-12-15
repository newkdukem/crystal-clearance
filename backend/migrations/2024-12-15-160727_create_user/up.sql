CREATE TABLE
  "user" (
    id uuid primary key,
    name varchar not null,
    email varchar not null,
    created_at timestamptz not null,
    updated_at timestamptz not null
  );