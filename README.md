# Team Activity

Tool to generate useless activity reports for management, based on Jira's activity streams.

## Usage

To start team-activity with given configuration use:

```bash
./team-activity config.yaml
```

By default without parameters use `config.yaml` file.

## Configuration Example

Following code contains simple configuration:

```yaml
streams:
  url: "https://jira.example.com/jira/plugins/servlet/streams"
  username: "IRobot"
  password: "Pa$$w0rd"

database:
  path: "local.sqlite"

server:
  bind_address: "localhost"
  bind_port: 8000

members:
  - BGates
  - JSmith
  - SJobs

activity:
  ignore:
    - application: com.atlassian.jira
      verbs:
        - "http://activitystrea.ms/schema/1.0/update"
        - "http://streams.atlassian.com/syndication/verbs/jira/transition"
        - "http://streams.atlassian.com/syndication/verbs/jira/stop"

    - application: com.atlassian.jira
      verbs:
        - "http://activitystrea.ms/schema/1.0/update"
        - "http://streams.atlassian.com/syndication/verbs/jira/transition"
        - "http://streams.atlassian.com/syndication/verbs/jira/close"

  activities:
    - application: com.atlassian.jira
      key: task::start
      group: ObjectIssue
      verbs: [ "http://activitystrea.ms/schema/1.0/update", "http://streams.atlassian.com/syndication/verbs/jira/transition", "http://streams.atlassian.com/syndication/verbs/jira/open" ]

    - application: com.atlassian.jira
      key: task::start
      group: ObjectIssue
      verbs: [ "http://activitystrea.ms/schema/1.0/update", "http://streams.atlassian.com/syndication/verbs/jira/transition", "http://streams.atlassian.com/syndication/verbs/jira/start" ]

    - application: com.atlassian.jira
      key: task::resolve
      group: ObjectIssue
      verbs: [ "http://activitystrea.ms/schema/1.0/update", "http://streams.atlassian.com/syndication/verbs/jira/transition", "http://streams.atlassian.com/syndication/verbs/jira/resolve" ]

    - application: com.atlassian.jira
      key: task::change
      group: ObjectIssue
      verbs: [ "http://activitystrea.ms/schema/1.0/update", "http://streams.atlassian.com/syndication/verbs/jira/transition" ]

    - application: com.atlassian.jira
      key: task::comment
      group: ObjectIssue
      verbs: [ "http://activitystrea.ms/schema/1.0/update" ]

    - application: com.atlassian.jira
      key: task::comment
      group: TargetIssue
      verbs: [ "http://activitystrea.ms/schema/1.0/post" ]

    - application: com.atlassian.jira
      key: task::start
      group: ObjectIssue
      verbs: [ "http://activitystrea.ms/schema/1.0/post" ]

  messages:
    task::start:
      - "Investigation root cause"
      - "Start implementation"
      - "Root cause analysis"

    task::resolve:
      - "Finished implementation"
      - "Submitted code"
      - "Committed code"
      - "Resolved issue"

    task::create:
      - "Created issue"
      - "New issue created"

    task::change:
      - "Local verification"
      - "Verification"
      - "Algorithm verification"
      - "Implementation"
      - "Algorithm implementation"

    task::comment:
      - "Local verification"
      - "Verification"
      - "Algorithm verification"
      - "Implementation"
      - "Algorithm implementation"

start_worker: true
pull_interval: 3600
```
