<!--
<img src="https://github.com/Jay-boo/InsightHoot/blob/feature-dashboard/Dashboard/dashboard/dashboard/static/insighthootlogo.png" alt="Size Limit CLI" width="100" align="right">
-->
# Grasper

[![GitHub Issues](https://img.shields.io/github/issues/Jay-boo/InsightHoot.svg?style=flat-square&label=Issues&color=d77982)](https://github.com/Jay-boo/InsightHoot/issues)
![Contributors](https://img.shields.io/github/contributors/Jay-boo/InsightHoot?style=flat-square)

[![Build Status](https://github.com/Jay-boo/InsightHoot/actions/workflows/sbt-scala-build.yaml/badge.svg)](https://github.com/Jay-boo/InsightHoot/actions/workflows/sbt-scala-build.yaml)
[![Build Status](https://github.com/Jay-boo/InsightHoot/actions/workflows/django.yaml/badge.svg)](https://github.com/Jay-boo/InsightHoot/actions/workflows/django.yaml)


Automatic monitoring project watching the latest updates in the tech field in a Kubernetes cluster.
The project is divided into 4 parts:
  - **Continuous RSS feed ingestion with Kafka**
  - **Daily batch processing with Spark structuring and tagging data**: For each received message, the goal is to obtain a list of tags describing the message's subject the best.
  - **Data visualization via a Django Dashboard**: Visualizing the evolution of tag appearances to detect the popularity of new tools.
  - **Team post automation**: Another purpose of the project is to be able to create posts summarizing the retrieved information effectively.

## Global

### RSS feed ingest

Currently explored RSS feeds include:

```{yaml}
     {
      "stackoverflow": "https://stackoverflow.blog/feed",
      "developpez": "https://cloud-computing.developpez.com/rss.php",
      "4sysops": "https://4sysops.com/feed/",
      "adamtheautomator": "https://adamtheautomator.com/feed/",
      "macmule": "https://macmule.com/feed/",
      "web3isgoinggreat": "https://www.web3isgoinggreat.com/feed",
      "thezvi": "https://thezvi.substack.com/feed",
      "zdnet": [
        "https://www.zdnet.com/topic/artificial-intelligence/rss.xml",
        "https://www.zdnet.com/topic/cloud/rss.xml",
        "https://www.zdnet.com/topic/digital-transformation/rss.xml"
      ],
      "databricks": "https://www.databricks.com/feed",
      "techtoday": "https://techtoday.co/category/tech-news/feed/",
      "reddittech": "https://www.reddit.com/r/technology/top.rss?t=day",
      "slashdot": "http://rss.slashdot.org/Slashdot/slashdotMain",
      "theverge": "https://www.theverge.com/rss/ai-artificial-intelligence/index.xml",
      "techcrunch": "https://techcrunch.com/feed/",
      "theguardian": "https://www.theguardian.com/us/technology/rss",
      "arstechnica": "https://feeds.arstechnica.com/arstechnica/technology-lab",
      "wsj": "https://feeds.a.dj.com/rss/RSSWSJD.xml",
      "ft": "https://www.ft.com/technology?format=rss",
      "dbta": "https://feeds.feedburner.com/DBTA-Articles"
    }
```

It's possible to easily add an RSS feed to listen to by adding it to the file `./InsightHoot/kind/k8s_config/kafka/kafka-connect.yaml` in the field `data/rss_topics.json`.


### Message tagging

Message tagging is done in 3 steps:
1. Retrieval of a Kafka message title.
2. Token filtering: Only `NN` tokens are retained. A pre-trained POS Tagger is used [pos_anc_en from johnsnowlabs](https://sparknlp.org/2021/03/05/pos_anc.html).
3. Use of Levenshtein distance to retrieve the closest tags.

A tag is characterized by a label and a theme, and the set of considered tags is described in the file `./InsightHoot/src/main/resources/tags.json`.


### Django Dashboard
[Coming soon...](https://github.com/Jay-boo/InsightHoot/tree/feature-dashboard)

### Teams post automation
Coming soon...

## Launch 🚀

The project is built for deployment on a Kubernetes cluster, meaning you can run it with `kind` using the `.create-kind-cluster.sh` script.

## Contributors

<a href="https://github.com/Jay-boo/InsightHoot/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Jay-boo/InsightHoot" />
</a>


