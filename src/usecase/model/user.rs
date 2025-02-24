use crate::domain::model::rules::RuleType;

pub struct UpdateUserData {
    pub user_name: Option<String>,
    pub icon_url: Option<String>,
    pub x_link: Option<String>,
    pub github_link: Option<String>,
    pub self_introduction: Option<String>,
}

impl UpdateUserData {
    pub fn validate(&self) -> anyhow::Result<()> {
        let rules = vec![
            (&self.user_name, RuleType::UserName),
            (&self.icon_url, RuleType::Icon),
            (&self.x_link, RuleType::XLink),
            (&self.github_link, RuleType::GitHubLink),
            (&self.self_introduction, RuleType::SelfIntroduction),
        ];
        for (value, rule) in rules {
            if let Some(value) = value {
                rule.validate(value)?;
            }
        }
        Ok(())
    }
}

pub struct UpdatePasswordData {
    pub old_password: String,
    pub new_password: String,
}

impl UpdatePasswordData {
    pub fn validate(&self) -> anyhow::Result<()> {
        RuleType::Password.validate(&self.old_password)?;
        RuleType::Password.validate(&self.new_password)?;
        Ok(())
    }
}
