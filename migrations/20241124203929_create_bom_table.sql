CREATE TABLE bom (
    project_id TEXT NOT NULL,
    part_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    PRIMARY KEY (project_id, part_id),
    FOREIGN KEY (project_id) REFERENCES project (id) ON DELETE CASCADE,
    FOREIGN KEY (part_id) REFERENCES part (id)
);
