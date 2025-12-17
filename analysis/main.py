import duckdb
import os

# main script for the analysis part.
# reads the json data and ships it to duckdb.

# --- Prerequisites ---
# needs the hackernews.json file to exist first

# file paths
DATA_DIR = "data"
DB_PATH = os.path.join(DATA_DIR, "hackernews.db")
JSON_PATH = os.path.join(DATA_DIR, "hackernews.jsonl")

# make sure the scraper actually ran first
if not os.path.exists(JSON_PATH):
    print(f"woops, can't find '{JSON_PATH}'")
    print("did you run the rust scraper yet?")
else:
    # create the db file if it's not there.
    con = duckdb.connect(database=DB_PATH, read_only=False)

    # read json into table.
    # use replace so i can just rerun this script without having to delete the db file.
    print("reading the json file...")
    con.execute(f"CREATE OR REPLACE TABLE stories AS SELECT * FROM read_json_auto('{JSON_PATH}')")

    # quick check
    result = con.execute("SELECT COUNT(*) FROM stories").fetchone()
    if result:
        story_count = result[0]
        print(f"looks good, {story_count} stories loaded into the 'stories' table.")
    else:
        # incase of error while reading table
        print("hmm, something went wrong, couldn't count the stories.")

    # close connection
    con.close()
