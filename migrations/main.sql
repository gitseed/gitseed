--liquibase formatted sql

--changeset paul:inital
CREATE TABLE presses (
  id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  date_pressed timestamp
);

