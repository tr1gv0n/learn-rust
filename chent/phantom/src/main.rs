use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64,Ordering},
};

static  NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free{
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free{
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature1 for {}",self.name);
    }

    fn feature2(&self) {
        println!("feature2 for {}",self.name);
    }
}

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}),enjoy this advanced feature!",
            self.name,self.id
        );
    }
}

pub struct  FreePlan;
pub struct PersonalPlan(f32);

impl<T> Customer<T>{
    pub fn new(name: String) -> Self{
        Self { id: NEXT_ID.fetch_add(1, Ordering::Relaxed), name, _type: PhantomData::default() }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(value: Customer<FreePlan>) -> Self {
        Self::new(value.name)
    }
}

pub fn subscribe(customer:Customer<FreePlan>,payment: f32)-> Customer<PersonalPlan>{
    let _plan = PersonalPlan(payment);

    customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        // 免费
        let customer = Customer::<FreePlan>::new("Tyr".into())
        // 使用免费feature
        customer.feature1();
        customer.feature2();
        // 付费
        let customer = subscribe(customer,6.99);
        customer.feature1();
        customer.feature2();
        //付费用户解锁新技能
        customer.advance_feature();
    }
}

// #[derive(Debug,Default,PartialEq,Eq)]
// pub struct Identifier<T> {
//     inner: u64,
//     _tag: PhantomData<T>,
// }

// #[derive(Debug,Default,PartialEq,Eq)]
// pub struct User {
//     id: Identifier<Self>,
// }

// #[derive(Debug,Default,PartialEq,Eq)]
// pub struct Product {
//     id: Identifier<Self>,
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn id_should_not_be_the_same() {
//         let user = User::default();
//         let product = Product::default();

//         // 两个id不能比较，因为她们属于不同的类型
//         // assert_ne!(user.id,product.id);

//         assert_eq!(user.id.inner,product.id.inner);
//     }
// }
fn main() {
    println!("Hello, world!");
}
