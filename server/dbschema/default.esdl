module default {
  scalar type Reason extending enum<Cheating, Griefing, Toxicity>;

  type User {
    required steam_id: str {
      constraint exclusive;
      readonly := true;
    }

    index on (.steam_id) {
      annotation title := 'User SteamID Index';
    }
  }

  type Report {
    required target: User;
    # Hash of the IP Address of the person who created the report.
    # This is a simple way to prevent report spamming. Storing SteamIDs
    # would not help in this case.
    author_hash: str;
  
    required reason: Reason;
    required timestamp: datetime {
      default := datetime_current();
    }

    # Only allow one report per (target, author_hash, reason) tuple.
    # Using the author_hash, we can mitigate report spamming from the same
    # person. This is not perfect, but it's the best we can do without
    # requiring complex user authentication mechanisms.
    constraint exclusive on ((
      .target,
      .author_hash,
      .reason,
    )) {
      errmessage := 'only one report per target and reason allowed';
    }

    index on (.target) {
      annotation title := 'Report Target Index';
    }
  }
}
