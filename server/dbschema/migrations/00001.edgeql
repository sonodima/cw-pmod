CREATE MIGRATION m1m673xo375zksvlqvjvwducgjm7ej2dsvwq3ofned4ui3uo4ic2pq
    ONTO initial
{
  CREATE TYPE default::User {
      CREATE REQUIRED PROPERTY steam_id: std::str {
          SET readonly := true;
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE INDEX ON (.steam_id) {
          CREATE ANNOTATION std::title := 'User SteamID Index';
      };
  };
  CREATE SCALAR TYPE default::Reason EXTENDING enum<Cheating, Griefing, Toxicity>;
  CREATE TYPE default::Report {
      CREATE REQUIRED LINK target: default::User;
      CREATE PROPERTY author_hash: std::str;
      CREATE REQUIRED PROPERTY reason: default::Reason;
      CREATE CONSTRAINT std::exclusive ON ((.target, .author_hash, .reason)) {
          SET errmessage := 'only one report per target and reason allowed';
      };
      CREATE INDEX ON (.target) {
          CREATE ANNOTATION std::title := 'Report Target Index';
      };
      CREATE REQUIRED PROPERTY timestamp: std::datetime {
          SET default := (std::datetime_current());
      };
  };
};
