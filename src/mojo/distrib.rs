use std::io::Error;
use std::io::ErrorKind;
use std::io;

enum DistributionFamilyType {
//    unknown = 0,
Bernoulli = 1,
//    modified_huber = 2,
//    multinomial = 3,
Gaussian = 4,
//    poisson = 5,
//    gamma = 6,
//    tweedie = 7,
//    huber = 8,
//    laplace = 9,
//    quantile = 10
}

impl DistributionFamilyType {
    fn parse(value: &str) -> io::Result<Box<DistributionFamily>> {
        if value.eq("bernoulli") { Ok(Box::new(BernoulliDistributionFamily{})) }
        else if value.eq("gaussian") { Ok(Box::new(GaussianDistributionFamily{})) }
        else { Err(Error::new(ErrorKind::InvalidData, format!("cannot parse distribution family type: '{}'", value))) }
    }
}

trait DistributionFamily {
    fn get_distribution_family_type(&self) -> DistributionFamilyType;
    fn link_inv(&self, f: f64) -> f64;
}

struct BernoulliDistributionFamily {}

impl DistributionFamily for BernoulliDistributionFamily {
    fn get_distribution_family_type(&self) -> DistributionFamilyType {DistributionFamilyType::Bernoulli }
    fn link_inv(&self, f: f64) -> f64 {1.0 / (1.0 + f64::exp(-f))}
}

struct GaussianDistributionFamily {}

impl DistributionFamily for GaussianDistributionFamily {
    fn get_distribution_family_type(&self) -> DistributionFamilyType {DistributionFamilyType::Gaussian }
    fn link_inv(&self, f: f64) -> f64 {f}
}

fn do_distribution(_preds: &[f64;2], _offset: f64) {

}

