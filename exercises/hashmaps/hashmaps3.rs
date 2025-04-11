// hashmaps3.rs
//
// 给定一个足球比赛的得分列表（每行一个）。每行的格式为：
// "<队伍1名称>,<队伍2名称>,<队伍1进球数>,<队伍2进球数>"
// 示例：England,France,4,2（英格兰进了4个球，法国进了2个球）。
//
// 你需要构建一个得分表，包含队伍名称、该队伍进球数和失球数。
// 构建得分表的一种方法是使用哈希映射。解决方案已部分使用哈希映射编写，
// 请完成它以通过测试。
//
// 让我通过测试！
//
// 执行 `rustlings hint hashmaps3` 或使用 `hint` watch 子命令获取提示。



use std::collections::HashMap;

// 一个用于存储队伍进球详情的结构体。
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // 队伍名称作为键，其关联的结构体作为值。
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO：使用从当前行提取的详细信息填充得分表。记住，team_1 的进球数
        // 将是 team_2 的失球数，同样地，team_2 的进球数将是 team_1 的失球数。


        // Ways 1 : 类cpp风格的对 HashMAp 中 键值对的值 进行的修改。
        // 第一步：获取 可变 迭代器 
        // 第二步：修改/添加 value
        // 处理team1的数据
        // if let Some(team_info) = scores.get_mut(&team_1_name) {
        //     team_info.goals_scored += team_1_score;
        //     team_info.goals_conceded += team_2_score;
        // } else {
        //     scores.insert(team_1_name, Team {goals_scored: team_1_score, goals_conceded: team_2_score});
        // }
        
        // 处理team2的数据
        // if let Some(team_info) = scores.get_mut(&team_2_name) {
        //     team_info.goals_scored += team_2_score;
        //     team_info.goals_conceded += team_1_score;
        // } else {
        //     scores.insert(team_2_name, Team {goals_scored: team_2_score, goals_conceded: team_1_score});
        // }

        // Ways 2: 使用 半链式调用 ，利用 or_insert() 执行失败自动返回 value 的引用这一设计，隐式获取 value 的可变引用。
        // let team_score_info = scores.entry(team_1_name).or_insert(Team {goals_scored : 0, goals_conceded :0});
        // (*team_score_info).goals_scored += team_1_score;
        // (*team_score_info).goals_conceded += team_2_score;
        // let team_score_info = scores.entry(team_2_name).or_insert(Team {goals_scored : 0, goals_conceded :0});
        // (*team_score_info).goals_scored += team_2_score;
        // (*team_score_info).goals_conceded += team_1_score;

        // Ways3 ：使用 Rust 倡导的 完全的 链式调用。使用 entry(T) 尝试 Key 值为 T 的Pair 是否存在。如果存在，然后 and_modify( Closure ) ，
        // 或者 or_insert () 
        scores.entry(team_1_name)
            .and_modify(|team_score| {team_score.goals_scored += team_1_score; team_score.goals_conceded += team_2_score;})
            .or_insert(Team {goals_scored : team_1_score, goals_conceded : team_2_score});
        scores.entry(team_2_name)
            .and_modify(|team_score| {team_score.goals_scored += team_2_score; team_score.goals_conceded += team_1_score;})
            .or_insert(Team {goals_scored : team_2_score, goals_conceded : team_1_score});


    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
