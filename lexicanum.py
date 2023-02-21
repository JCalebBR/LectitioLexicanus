import requests
import json
from tqdm import tqdm

# Set up the API endpoint and query parameters
api_endpoint = "https://wh40k.lexicanum.com/mediawiki/api.php"
params = {
    "action": "query",
    "format": "json",
    "prop": "redirects|info|extracts",
    "rawcontinue": 1,
    "generator": "allpages",
    "rdprop": "title|pageid",
    "rdlimit": "max",
    "inprop": "url",
    "exlimit": "max",
    "exintro": 1,
    "explaintext": 1,
    "gapcontinue": "",
    "gapfilterredir": "nonredirects",
    "gaplimit": "20",
}

# Send the API requests and save output to file
with open("output.jsonl", "a+") as output_file:

    with tqdm() as pbar:
        while True:
            pbar.update(1)
            # Send the API request and get the response
            response = requests.get(api_endpoint, params=params)
            data = response.json()
            # Extract the pages from the response and output them in JSONL format
            pages = data["query"]["pages"].values()
            for page in pages:
                output = {
                    "title": page["title"],
                    "pageid": page["pageid"],
                    "extract": page["extract"],
                }
                if "redirects" in page:
                    output.update({"redirects": page["redirects"]})
                output_file.write(json.dumps(output) + "\n")

            # Check if there are more pages to retrieve
            if "gapcontinue" in data["query-continue"]["allpages"]:
                params["gapcontinue"] = data["query-continue"]["allpages"]["gapcontinue"]
            else:
                break
