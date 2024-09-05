# Matchday Backend Developer Take Home Assignment

## Your Mission

You are an aspiring football talent scout, who also happens to be technically savvy and a talented Rust developer!
Put your skills into use, and find out a way to track the best performers in football.

## The Task

### 1. Set up Sportradar Soccer API

- Signup for Sportradar API here, or use an existing account.
  https://console.sportradar.com/signup
- Follow the registration steps.
- Once logged in, select _Add Trials_, and pick Soccer API from the list.

### 2. Tool Implementation

- Build a command line program, which will look at Premier League's season 23/24 and output
- the 10 players who scored the most, and
- the 10 players who assisted the most.

#### NOTES
1) You may find these endpoints useful for the implementation
   - https://developer.sportradar.com/soccer/reference/soccer-competition-seasons
   - https://developer.sportradar.com/soccer/reference/soccer-season-schedule
   - https://developer.sportradar.com/soccer/reference/soccer-league-timeline
   - https://developer.sportradar.com/soccer/reference/soccer-season-competitors
   - https://developer.sportradar.com/soccer/reference/soccer-seasonal-competitor-statistics

    
2) There is one endpoint that returns the top performers for a season directly. We ask you not to use that one in your solution. You may use it for double-checking your results, though: https://developer.sportradar.com/soccer/reference/soccer-adv-analytics-season-leaders

3) We hope to see `tokio` and `reqwest` in the mix, other than that feel free to pick any crates you like.
4) Key aspect we look forward to see in the solution:
   - Composition,
   - Simplicity,
   - Readability,
   - Configurability / Extensibility of the solution,
   - Testability,
   - Error handling, but it can be on the simple side,
   - Tests and code comments are welcome, where it makes sense.
5) We don't expect you to spend more than a few hours on the implementation, so it's fine to give priority to the aspects you consider more important.


### 3. Deliver your solution
- A README is expected, so that we know how to run your code.
- Once you are happy with your solution, you can either:
- send it as a ZIP file via email to [matilda@matchday.com](mailto:matilda@matchday.com) and [matteo@matchday.com](mailto:matteo@matchday.com), or
- if you have it as a private repository on GitHub, invite the following usernames as collaborators:
- `matildasmeds`
- `sardusmatt`

## Timeline and Next Steps
- We know everyone is busy, so we give a week to complete the task. If for any reason you need more time, please let us know. If anything is unclear / incorrect in the task spec, please send us an email and we'll amend.
- Once we have the solution, we are going to review it, and ideally schedule the following interview within 7 days, depending on availability on both sides.
