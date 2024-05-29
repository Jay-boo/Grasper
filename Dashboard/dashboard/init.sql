-- Create the topics table with the new structure
CREATE TABLE topics (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    url VARCHAR(255) UNIQUE NOT NULL
);

-- Create the tags table with a unique constraint on (label, theme)
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    label VARCHAR(255) NOT NULL,
    theme VARCHAR(255) NOT NULL,
    UNIQUE (label, theme)
);

-- Create the messages table with the new structure
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    date TIMESTAMP NOT NULL,
    content TEXT,
    title VARCHAR(400) NOT NULL,
    link VARCHAR(400) NOT NULL,
    topic_id INT NOT NULL,
    FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE CASCADE
);

CREATE TABLE message_tags (
    messageId INT NOT NULL,
    tagId INT NOT NULL,
    FOREIGN KEY (messageId) REFERENCES messages(id) ON DELETE CASCADE,
    FOREIGN KEY (tagId) REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (messageId, tagId)
);

INSERT INTO topics (name, url) VALUES 
    ('4sysops', 'http://4sysops.com/feed/'),
    ('theguardian', 'https://www.theguardian.com/us/technology/rss'),
    ('reddittech', 'https://www.reddit.com/r/technology/top.rss?t=day'),
    ('developpez', 'https://cloud-computing.developpez.com/rss.php'),
    ('wsj', 'https://feeds.a.dj.com/rss/RSSWSJD.xml'),
    ('slashdot', 'http://rss.slashdot.org/Slashdot/slashdotMain');

INSERT INTO tags (label, theme) VALUES 
    ('Java', 'langages'),
    ('Python', 'langages'),
    ('JavaScript', 'langages'),
    ('C++', 'langages'),
    ('C#', 'langages'),
    ('Ruby', 'langages'),
    ('Swift', 'langages'),
    ('Kotlin', 'langages'),
    ('TypeScript', 'langages'),
    ('PHP', 'langages'),
    ('Go', 'langages'),
    ('Perl', 'langages'),
    ('Shell', 'langages'),
    ('HTML', 'langages'),
    ('CSS', 'langages'),
    ('Objective-C', 'langages'),
    ('Dart', 'langages'),
    ('AI', 'IA'),
    ('ML', 'IA'),
    ('DL', 'IA'),
    ('NLP', 'IA'),
    ('CV', 'IA'),
    ('NN', 'IA'),
    ('RL', 'IA'),
    ('Chatbots', 'IA'),
    ('VA', 'IA'),
    ('Ethics', 'IA'),
    ('Gov', 'IA'),
    ('Ubuntu', 'OS');

INSERT INTO messages (date, content, title, link, topic_id) VALUES 
    ('2024-05-10T15:17:50Z', 'Random content', 'Random Title', 'https://example.com/random-link', 1),
    ('2024-05-02T16:29:12Z', 'Random content', 'Random Title', 'https://example.com/random-link', 1),
    ('2024-05-10T21:06:52Z', 'Random content', 'Random Title', 'https://example.com/random-link', 2),
    ('2024-05-12T15:42:42Z', 'Random content', 'Random Title', 'https://example.com/random-link', 3),
    ('2024-05-12T13:04:46Z', 'Random content', 'Random Title', 'https://example.com/random-link', 3),
    ('2024-05-08T03:05:00Z', 'Random content', 'Random Title', 'https://example.com/random-link', 4),
    ('2024-04-26T04:58:00Z', 'Random content', 'Random Title', 'https://example.com/random-link', 5),
    ('2024-04-25T10:15:00Z', 'Random content', 'Random Title', 'https://example.com/random-link', 1),
    ('2024-04-24T08:22:00Z', 'Random content', 'Random Title', 'https://example.com/random-link', 2),
    ('2024-04-23T15:47:00Z', 'Random content', 'Random Title', 'https://example.com/random-link', 3);

INSERT INTO message_tags (messageId, tagId) VALUES 
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 2),
    (5, 18),
    (6, 29),
    (6, 15),
    (7, 3);
