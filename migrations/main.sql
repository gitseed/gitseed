--liquibase formatted sql

--changeset paul:inital
CREATE TABLE github_events (
  id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  ts timestamp,
  github_org_name text,
  event_type text,
  payload jsonb
);
