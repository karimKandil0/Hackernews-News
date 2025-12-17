import duckdb
import os

DB_PATH = os.path.join("data", "hackernews.db")

try:
    # Connect to the DuckDB database
    con = duckdb.connect(database=DB_PATH, read_only=True) # read_only=True for inspection

    print(f"Connected to DuckDB database: {DB_PATH}")

    # List all tables in the database
    print("\n--- Tables in the database ---")
    tables = con.execute("SHOW TABLES;").fetchall()
    for table in tables:
        print(table[0])

    # Describe the schema of the 'stories' table
    print("\n--- Schema of 'stories' table ---")
    schema = con.execute("DESCRIBE stories;").fetchall()
    for row in schema:
        print(f"Column: {row[0]}, Type: {row[1]}, Null: {row[2]}")

    # Fetch and print a few rows from the 'stories' table
    print("\n--- First 5 rows from 'stories' table ---")
    sample_data = con.execute("SELECT title, author, score FROM stories LIMIT 5;").fetchall()
    for row in sample_data:
        print(f"Title: {row[0]}, Author: {row[1]}, Score: {row[2]}")

    # You can run any SQL query here, for example, to check for duplicates by URL:
    print("\n--- Checking for duplicates by URL in DB ---")
    duplicate_urls = con.execute("SELECT url, COUNT(*) FROM stories GROUP BY url HAVING COUNT(*) > 1;").fetchall()
    if duplicate_urls:
        print("Found duplicates:")
        for url, count in duplicate_urls:
            print(f"  URL: {url}, Count: {count}")
    else:
        print("No duplicate URLs found in the 'stories' table.")

except duckdb.Error as e:
    print(f"An error occurred: {e}")
finally:
    if 'con' in locals() and con:
        con.close()
        print("\nDuckDB connection closed.")
