# Connecting to PostgreSQL

## Prerequisites

- `kubectl` configured with cluster access
- `psql` client installed (or use pod exec)

## Method 1: Port Forward + psql

```bash
# Terminal 1: Start port forward
kubectl port-forward svc/auth-postgres 5432:5432

# Terminal 2: Connect with psql
psql -h localhost -U postgres -d authdb
# Password: postgres
```

## Method 2: Exec into Pod

```bash
# Get pod name
kubectl get pods -l app=auth-postgres

# Exec into pod
kubectl exec -it <pod-name> -- psql -U postgres -d authdb

# Or one-liner
kubectl exec -it $(kubectl get pods -l app=auth-postgres -o jsonpath='{.items[0].metadata.name}') -- psql -U postgres -d authdb
```

## psql Meta Commands

| Command | Description |
|---------|-------------|
| `\l` | List all databases |
| `\c dbname` | Connect to a database |
| `\dt` | List tables in current database |
| `\d tablename` | Describe table structure |
| `\du` | List users/roles |
| `\dn` | List schemas |
| `\di` | List indexes |
| `\df` | List functions |
| `\x` | Toggle expanded output |
| `\timing` | Toggle query timing |
| `\q` | Quit psql |

## Database Operations

```sql
-- Create database
CREATE DATABASE mydb;

-- Drop database
DROP DATABASE mydb;

-- Connect to database
\c authdb
```

## Table Operations

```sql
-- Create table
CREATE TABLE example (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Drop table
DROP TABLE example;

-- Rename table
ALTER TABLE old_name RENAME TO new_name;
```

## CRUD Operations

### Insert

```sql
-- Single row
INSERT INTO users (email, hashed_password, first_name)
VALUES ('user@example.com', 'hashed_pw', 'John');

-- Multiple rows
INSERT INTO users (email, hashed_password, first_name) VALUES
    ('user1@example.com', 'hash1', 'Alice'),
    ('user2@example.com', 'hash2', 'Bob');
```

### Select

```sql
-- Select all
SELECT * FROM users;

-- Select specific columns
SELECT id, email, first_name FROM users;

-- With conditions
SELECT * FROM users WHERE is_active = true;

-- With sorting
SELECT * FROM users ORDER BY created_at DESC;

-- With limit
SELECT * FROM users LIMIT 10 OFFSET 0;

-- Count rows
SELECT COUNT(*) FROM users;
```

### Update

```sql
-- Update single row
UPDATE users SET first_name = 'Jane' WHERE id = '123';

-- Update multiple columns
UPDATE users SET first_name = 'Jane', last_name = 'Doe' WHERE email = 'user@example.com';

-- Update all rows
UPDATE users SET is_active = false;
```

### Delete

```sql
-- Delete specific rows
DELETE FROM users WHERE id = '123';

-- Delete with condition
DELETE FROM users WHERE is_active = false;

-- Delete all rows (use with caution)
DELETE FROM users;

-- Truncate (faster for large tables)
TRUNCATE TABLE users;
```

## Useful Queries

```sql
-- Check table size
SELECT pg_size_pretty(pg_total_relation_size('users'));

-- List active connections
SELECT * FROM pg_stat_activity;

-- Show current database
SELECT current_database();

-- Show current user
SELECT current_user;

-- Show server version
SELECT version();
```
