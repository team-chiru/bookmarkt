DROP TABLE node;

CREATE TABLE IF NOT EXISTS node (
   id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL,
   type TEXT NOT NULL,
   node_id TEXT NOT NULL,
   node_level TEXT,
   rev_no INTEGER NOT NULL,

   FOREIGN KEY(node_id) REFERENCES node(node_id) ON DELETE CASCADE
);
