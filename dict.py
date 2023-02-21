import json

with open("dict/content.html", "w", encoding="utf-8") as content, open(
    "filtered-output.jsonl", "r"
) as words:
    content.write(
        """<html xmlns:math="http://exslt.org/math" xmlns:svg="http://www.w3.org/2000/svg"
    xmlns:tl="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf"
    xmlns:saxon="http://saxon.sf.net/" xmlns:xs="http://www.w3.org/2001/XMLSchema"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:cx="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf"
    xmlns:dc="http://purl.org/dc/elements/1.1/"
    xmlns:mbp="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf"
    xmlns:mmc="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf"
    xmlns:idx="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf">\n"""
    )

    content.write(
        """
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <style>
        h5 {
            font-size: 1em;
            margin: 0;
        }

        dt {
            font-weight: bold;
        }

        dd {
            margin: 0;
            padding: 0 0 0.5em 0;
            display: block
        }
    </style>
</head>\n"""
    )
    content.write(
        """<body>
    <mbp:frameset>"""
    )

    for line in words:
        line = line.encode("ascii", "xmlcharrefreplace").decode("utf-8")
        word = json.loads(line)
        # if page content isn't empty
        infls = []
        nl = "\n\t\t\t\t\t\t\t\t"
        try:
            if word["redirects"]:
                for redirect in word["redirects"]:
                    print(redirect["title"])
                    infls.append(redirect["title"])
        except KeyError:
            pass
        content.write(
            f"""
            <idx:entry name="default" scriptable="yes" spell="yes">
                <h5>
                    <dt>
                        <idx:orth>{word["title"]}
                            {"<idx:infl>" if infls else ""}
                                {nl.join([f'''<idx:iform value="{infl}" />''' for infl in infls])}
                            {"<idx:infl>" if infls else ""}
                        </idx:orth>
                    </dt>
                </h5>
                <dd>{word["extract"]}</dd>
            </idx:entry>
            <hr />
            """
        )
    content.write(
        """
            </mbp:frameset>
        </body>
        """
    )

