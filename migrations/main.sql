--liquibase formatted sql

--changeset paul:1
CREATE TABLE cars (
  brand VARCHAR(255),
  model VARCHAR(255),
  year INT
);
