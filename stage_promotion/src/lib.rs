#[derive(PartialEq, Debug)]
pub struct Stage<T> {
    pub name: String,
    pub deployment: T,
}

pub trait StagePromotion<T: Copy> {
    fn promote(&self, from: &Stage<T>, to: &Stage<T>) -> Stage<T>;
}

pub struct MockingDeployment;

impl StagePromotion<i32> for MockingDeployment {
    fn promote(&self, from: &Stage<i32>, to: &Stage<i32>) -> Stage<i32> {
        Stage {
            name: to.name.clone(),
            deployment: from.deployment.clone(),
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
            deployment: 1,
        };
    }

    #[test]
    fn should_equal_from_stage() {
        let deployment = MockingDeployment;
        let dev = Stage {
            name: String::from("dev"),
            deployment: 1,
        };
        let prod = Stage {
            name: String::from("prod"),
            deployment: 2,
        };

        let promoted_prod_stage = deployment.promote(&dev, &prod);

        assert_eq!(prod.name, promoted_prod_stage.name);
        assert_eq!(dev.deployment, promoted_prod_stage.deployment)
    }
}
