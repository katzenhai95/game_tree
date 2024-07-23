pub mod game_tree {
    use std::cmp::Ordering;

    struct GameTreeNode<T>
    where
        T: SituationOps,
    {
        data: Option<T>,
        cur_move: Option<T::Move>,
        is_max_layer: bool,
        cost: i32,
        children: Vec<GameTreeNode<T>>,
    }

    impl<T> GameTreeNode<T>
    where
        T: SituationOps,
    {
        fn new(data: Option<T>, cur_move: Option<T::Move>, is_max_layer: bool) -> Self {
            Self {
                data: data,
                cur_move: cur_move,
                is_max_layer: is_max_layer,
                cost: 0,
                children: Vec::with_capacity(0),
            }
        }
    }

    pub trait SituationOps {
        type Move;
        fn get_avilable_move(&self) -> Vec<Self::Move>;
        fn proc_move(&mut self, next_move: &Self::Move);
        fn with_move(&self, next_move: &Self::Move) -> Self;
        fn calc_cost(&self) -> i32;
    }

    pub struct GameTree<T>
    where
        T: SituationOps,
    {
        search_depth: u32,
        cur_situation: T,
    }

    impl<T> GameTree<T>
    where
        T: SituationOps,
    {
        pub fn new(search_depth: u32, init_situation: T) -> Self {
            Self {
                search_depth: search_depth,
                cur_situation: init_situation,
            }
        }

        pub fn get_cur_situation(&self) -> &T {
            &self.cur_situation
        }

        pub fn proc_move(&mut self, next_move: &T::Move) {
            self.cur_situation.proc_move(next_move);
        }

        pub fn get_next_move(&self) -> Option<T::Move> {
            let mut root = GameTreeNode::new(None, None, true);
            self.build_game_tree(&mut root, self.search_depth);
            match self.min_max_search(&mut root) {
                Some(idx) => Some(root.children.remove(idx).cur_move.unwrap()),
                None => None,
            }
        }

        fn build_game_tree(&self, root: &mut GameTreeNode<T>, search_depth: u32) {
            if search_depth == 0 {
                return;
            }
            let cur_situation = match &root.data {
                Some(data) => &data,
                None => &self.cur_situation,
            };
            root.children = Vec::from_iter(cur_situation.get_avilable_move().into_iter().map(
                |next_move| {
                    GameTreeNode::new(
                        Some(cur_situation.with_move(&next_move)),
                        Some(next_move),
                        !root.is_max_layer,
                    )
                },
            ));
            for child in &mut root.children {
                self.build_game_tree(child, search_depth - 1);
            }
        }

        fn min_max_search<'a>(&self, root: &mut GameTreeNode<T>) -> Option<usize> {
            let cur_situation = match &root.data {
                Some(data) => &data,
                None => &self.cur_situation,
            };
            if root.children.is_empty() {
                root.cost = cur_situation.calc_cost();
                return None;
            }
            for child in &mut root.children {
                self.min_max_search(child);
            }
            let mut next_move_id = 0;
            let cmp_res = match root.is_max_layer {
                true => Ordering::Greater,
                false => Ordering::Less,
            };
            for child in root.children.iter().enumerate() {
                if child.1.cost.cmp(&root.children[next_move_id].cost) == cmp_res {
                    next_move_id = child.0;
                }
            }
            root.cost = root.children[next_move_id].cost;
            Some(next_move_id)
        }

        #[allow(unused)]
        fn alpha_beta_search(
            &self,
            root: &GameTreeNode<T>,
            cost_threshold: Option<i32>,
        ) -> Option<usize> {
            todo!()
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
