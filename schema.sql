-- Create an enumerated type for event types
CREATE TYPE event_type AS ENUM ('begin_block', 'end_block', 'finalize_block', 'tx');

-- Create the main table for storing address mentions
CREATE TABLE address_mentions (
    id BIGSERIAL PRIMARY KEY,
    address TEXT NOT NULL,
    block_height BIGINT NOT NULL,
    tx_hash TEXT,
    event_type event_type NOT NULL,
    event_index INT,
    attribute_key TEXT,
    attribute_value TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_address_mentions_address ON address_mentions(address);

CREATE INDEX idx_address_mentions_address_block_height ON address_mentions(address, block_height);

CREATE TABLE blocks (
    height BIGINT PRIMARY KEY,
    app_hash TEXT NOT NULL,
    validator_updates JSONB,
    consensus_param_updates JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tx_results (
    id BIGSERIAL PRIMARY KEY,
    block_height BIGINT NOT NULL,
    tx_index INT NOT NULL,
    result JSONB NOT NULL,
    FOREIGN KEY (block_height) REFERENCES blocks(height)
);

CREATE TABLE transactions (
    tx_hash TEXT PRIMARY KEY,
    block_height BIGINT NOT NULL,
    tx_index INT NOT NULL,
    tx_data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (block_height) REFERENCES blocks(height)
);


CREATE TABLE block_events (
    id BIGSERIAL PRIMARY KEY,
    block_height BIGINT NOT NULL,
    event_type event_type NOT NULL,
    event_index INT NOT NULL,
    event_data JSONB NOT NULL,
    FOREIGN KEY (block_height) REFERENCES blocks(height)
);

SELECT DISTINCT t.tx_hash, t.block_height
FROM transactions t
JOIN address_mentions am ON t.tx_hash = am.tx_hash
WHERE am.address = 'address1';

SELECT DISTINCT block_height
FROM address_mentions
WHERE address = 'address1';

Explain why these 2 indexes are needed:
Both indexes are needed to optimize different types of queries on the address_mentions table. It improves query performance by allowing the database to quickly locate rows based on the address column or a combination of address and block_height columns.
Improve performance while the second index can cover most use cases of the first index, keeping both can be beneficial for query performance in different scenarios.
Having both indexes gives the database query planner more options to choose from when optimizing queries.


Bloom filter index" and query that can benefit from such index?
CREATE INDEX bloom_idx ON address_mentions USING bloom (address);

Example query that can benefit from a Bloom filter index:
SELECT * FROM address_mentions 
WHERE address = 'address1' AND tx_hash = 'hash1' AND event_type = 'tx';

Bloom filters use less space than traditional B-tree indexes, especially for high-cardinality columns.
Good for multi-column conditions: Can efficiently handle queries with multiple equality conditions on different columns.

So I rewrite ideas for further optimizations:
-- Denormalization
-- BRIN (Block Range) Indexes
-- Clustering and Caching Query/Database Results