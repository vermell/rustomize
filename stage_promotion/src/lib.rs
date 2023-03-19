#[derive(PartialEq, Debug)]
pub struct Stage {
    pub name: String,
}

pub trait StagePromotion {
    fn promote(&self, from: &Stage, to: &Stage) -> Stage;
}

struct MockingDeployment;

impl StagePromotion for MockingDeployment {
    fn promote(&self, from: &Stage, to: &Stage) -> Stage {
        Stage {
            name: from.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_initialize_stage() {
        Stage {
            name: String::from("dev"),
        };
    }

    #[test]
    fn should_equal_from_stage() {
        let deployment = MockingDeployment;
        let dev = Stage {
            name: String::from("dev"),
        };
        let prod = Stage {
            name: String::from("prod"),
        };

        let promoted_prod_stage = deployment.promote(&dev, &prod);

        assert_eq!(dev, promoted_prod_stage);
    }
}
