CREATE TABLE Tags (
    tid AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE Posts (
    pid AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    views INT DEFAULT 0,
    summary TEXT,
    status VARCHAR(7) DEFAULT 'PUBLIC' CHECK (status IN ('PRIVATE', 'PUBLIC', 'DELETED'))
);

CREATE TABLE PostTags (
    post_id INT,
    tag_id INT,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES Posts(pid),
    FOREIGN KEY (tag_id) REFERENCES Tags(tid)
);

CREATE TABLE Users (
    uid AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    role VARCHAR(5) DEFAULT 'USER' CHECK (role IN ('USER', 'OWNER'))
);

CREATE TABLE Comments (
    cid AUTO_INCREMENT PRIMARY KEY,
    parent_cid INT,
    username VARCHAR(255),
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    status VARCHAR(7) DEFAULT 'PUBLIC' CHECK (status IN ('PRIVATE', 'PUBLIC', 'DELETED')),
    FOREIGN KEY (parent_cid) REFERENCES Comments(cid),
    FOREIGN KEY (username) REFERENCES Users(username)
);

---

INSERT INTO Users (username, role) VALUES ('YiNNx', 'OWNER');

INSERT INTO Tags (name) VALUES ('Programming'), ('Life');
