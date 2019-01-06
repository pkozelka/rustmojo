use std::io::Error;
use std::io::ErrorKind;
use std::io;

pub enum DistributionFamilyType {
//    unknown = 0,
Bernoulli = 1,
ModifiedHuber = 2, // not implemented
Multinomial = 3, // not implemented
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
        match value {
            "bernoulli" => Ok(Box::new(BernoulliDistributionFamily {})),
            "gaussian" => Ok(Box::new(GaussianDistributionFamily{})),
            _ => Err(Error::new(ErrorKind::InvalidData, format!("cannot parse distribution family type: '{}'", value)))
        }
    }
}

pub trait DistributionFamily {
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

pub fn do_distribution(preds: &mut[f64;3], init_f: f64, offset: f64, n_classes: usize, family: &DistributionFamily) -> io::Result<()> {
    match family.get_distribution_family_type() {
        DistributionFamilyType::Bernoulli | DistributionFamilyType::ModifiedHuber/*NI*/ => {
            let f = preds[1] + init_f + offset;
            preds[2] = family.link_inv(f);
            preds[1] = 1.0 - preds[2];
        },
        DistributionFamilyType::Multinomial => {
            if n_classes == 2 {
                preds[1] += init_f + offset;
                preds[2] = -preds[1];
            } else {
                return Err(Error::new(ErrorKind::InvalidData, format!("multinomial unimplemented (n_classes = {})", n_classes)));
            }
        },
        DistributionFamilyType::Gaussian => {
            // Regression
            let f = preds[0] + init_f + offset;
            preds[0] = family.link_inv(f);
        },
    }
    Ok(())
}

#[test]
fn test_distrib() {
    let x = DistributionFamilyType::parse("bernoulli").unwrap();
    assert_eq!(DistributionFamilyType::Bernoulli as usize, x.get_distribution_family_type() as usize);
}