import json
import re

# Define the patterns to filter the titles
patterns = [
    r"\((disambiguation|song|novel|anthology|novella|list|game)\)",
    r"\b(miniatures|magazine|white dwarf)\b",
]
filters = [re.compile(p, flags=re.IGNORECASE) for p in patterns]

# Open the input and output files
with open("dump.jsonl", "r") as input_file, open(
    "filtered-output.jsonl", "w", encoding="ascii"
) as output_file:
    # Iterate through the input file and write filtered data to the output file
    for line in input_file:
        # Parse the JSON object from the line
        line = line.encode("ascii", errors="xmlcharrefreplace").decode("utf-8")
        data = json.loads(line)
        # if data["extract"] has a number inside [], remove said number and brackets from extract
        if "[" in data["extract"]:
            data["extract"] = re.sub(r"\[.*?\]", "", data["extract"])
        # Filter the title based on the patterns
        if any(filter.search(data["title"]) for filter in filters):
            continue  # Skip this line

        # Write the filtered data to the output file
        output_file.write(json.dumps(data) + "\n")
