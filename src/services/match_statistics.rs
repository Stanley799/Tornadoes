//! Service for computing match and player statistics for handball matches.
//! Uses SQLx for database queries and aggregates stats efficiently.

use sqlx::PgPool;
use serde::Serialize;

/// Team-level statistics for a match.
/// Includes goals, fast breaks, penalties, saves, and defense events for both teams.
#[derive(Serialize)]
pub struct TeamStatistics {
    pub home_team_goals: i32,
    pub away_team_goals: i32,
    pub home_team_fast_break_goals: i32,
    pub away_team_fast_break_goals: i32,
    pub home_team_penalty_goals: i32,
    pub away_team_penalty_goals: i32,
    pub home_team_goalkeeper_saves: i32,
    pub away_team_goalkeeper_saves: i32,
    pub home_team_defense_events: i32,
    pub away_team_defense_events: i32,
}

/// Player-level statistics for a match.
/// Only tracks goals scored per player, with name and team info.
#[derive(Serialize)]
pub struct PlayerStatistics {
    pub player_id: i64,
    pub name: String,
    pub team: String, // "home" or "away"
    pub goals: i32,
}

/// Aggregated statistics response for a match.
/// Contains team-level and player-level stats, match identifiers, and result.
#[derive(Serialize)]
pub struct MatchStatisticsResponse {
    pub match_id: i64,
    pub tournament_id: Option<i64>,
    pub season_id: Option<i64>,
    pub home_team: String,
    pub away_team: String,
    pub result: TeamResult,
    pub team_statistics: TeamStatistics,
    pub players: Vec<PlayerStatistics>,
}

/// Result for each team (win/loss/draw).
/// Used to indicate outcome for home and away teams.
#[derive(Serialize)]
pub struct TeamResult {
    pub home_team: String,
    pub away_team: String,
}

/// Compute statistics for a match by match_id.
/// Returns None if match does not exist.
/// Returns error for DB issues.
///
/// # Team-level vs Player-level
/// - Team-level: goals, fast breaks, penalties, saves, defense events.
/// - Player-level: only goals, with player name and team.
pub async fn compute_match_statistics(pool: &PgPool, match_id: i64) -> Result<Option<MatchStatisticsResponse>, sqlx::Error> {
    // Get match info
    let match_row = sqlx::query!(
        r#"SELECT id, tournament_id, season_id, home_team, away_team, home_score, away_score FROM matches WHERE id = $1"#,
        match_id
    ).fetch_optional(pool).await?;
    let match_row = match match_row {
        Some(row) => row,
        None => return Ok(None),
    };

    // Get all events for this match, joined with players and match info
    let events = sqlx::query!(
        r#"
        SELECT e.*, p.first_name, p.last_name,
            CASE WHEN e.match_id = m.id AND m.home_team = (SELECT home_team FROM matches WHERE id = $1) THEN 'home' ELSE 'away' END as team
        FROM match_events e
        JOIN players p ON e.player_id = p.id
        JOIN matches m ON e.match_id = m.id
        WHERE e.match_id = $1
        "#,
        match_id
    ).fetch_all(pool).await?;

    // Team-level aggregation
    let mut home_team_goals = 0;
    let mut away_team_goals = 0;
    let mut home_team_fast_break_goals = 0;
    let mut away_team_fast_break_goals = 0;
    let mut home_team_penalty_goals = 0;
    let mut away_team_penalty_goals = 0;
    let mut home_team_goalkeeper_saves = 0;
    let mut away_team_goalkeeper_saves = 0;
    let mut home_team_defense_events = 0;
    let mut away_team_defense_events = 0;

    let mut player_stats = std::collections::HashMap::new();

    for e in &events {
        let team = match &e.team {
            Some(t) if t == "home" => "home",
            _ => "away",
        };
        // Team-level stats
        match e.event_type.as_str() {
            "goal" => {
                if team == "home" { home_team_goals += 1; } else { away_team_goals += 1; }
                if e.is_fast_break {
                    if team == "home" { home_team_fast_break_goals += 1; } else { away_team_fast_break_goals += 1; }
                }
                if e.is_penalty {
                    if team == "home" { home_team_penalty_goals += 1; } else { away_team_penalty_goals += 1; }
                }
            },
            "save" => {
                if team == "home" { home_team_goalkeeper_saves += 1; } else { away_team_goalkeeper_saves += 1; }
            },
            "block" | "steal" => {
                if team == "home" { home_team_defense_events += 1; } else { away_team_defense_events += 1; }
            },
            _ => {}
        }
        // Player-level stats
        if e.event_type == "goal" {
            let entry = player_stats.entry(e.player_id).or_insert((format!("{} {}",
                e.first_name.as_str(),
                e.last_name.as_str()),
                team,
                0));
            entry.2 += 1;
        }
    }

    // Determine result
    let result = if home_team_goals > away_team_goals {
        TeamResult { home_team: "win".to_string(), away_team: "loss".to_string() }
    } else if home_team_goals < away_team_goals {
        TeamResult { home_team: "loss".to_string(), away_team: "win".to_string() }
    } else {
        TeamResult { home_team: "draw".to_string(), away_team: "draw".to_string() }
    };

    // Build player stats vector
    let mut players = Vec::new();
    for (player_id, (name, team, goals)) in player_stats {
        players.push(PlayerStatistics {
            player_id,
            name,
            team: team.to_string(),
            goals,
        });
    }

    Ok(Some(MatchStatisticsResponse {
        match_id: match_row.id,
        tournament_id: match_row.tournament_id,
        season_id: match_row.season_id,
        home_team: match_row.home_team,
        away_team: match_row.away_team,
        result,
        team_statistics: TeamStatistics {
            home_team_goals,
            away_team_goals,
            home_team_fast_break_goals,
            away_team_fast_break_goals,
            home_team_penalty_goals,
            away_team_penalty_goals,
            home_team_goalkeeper_saves,
            away_team_goalkeeper_saves,
            home_team_defense_events,
            away_team_defense_events,
        },
        players,
    }))
}
