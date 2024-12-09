-- init.sql
CREATE DATABASE IF NOT EXISTS new_db;
USE new_db;

CREATE TABLE Post (
    id INT AUTO_INCREMENT PRIMARY KEY,
    item VARCHAR(255) NOT NULL
);

INSERT INTO Post (item) VALUES ('First Item'), ('Second Item');
