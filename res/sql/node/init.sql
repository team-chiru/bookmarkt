DROP TABLE node;

CREATE TABLE IF NOT EXISTS node (
   id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   node_id INTEGER NOT NULL,
   name TEXT NOT NULL,
   rev_no INTEGER NOT NULL,

   FOREIGN KEY(node_id) REFERENCES node(node_id)
);