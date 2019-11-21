const BASES: [(char, char); 4] = [('A', 'U'), ('G', 'C'), ('T', 'A'), ('C', 'G')];

enum BASE_TYPE {
    DNA = 0,
    RNA = 1,
}

#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

fn get_bases(baseType: BASE_TYPE) -> Vec<char> {
    BASES
        .into_iter()
        .map(|t| match &baseType {
            BASE_TYPE::DNA => t.0,
            BASE_TYPE::RNA => t.1,
        })
        .collect::<Vec<char>>()
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let invalid: Option<usize> = dna.chars().position(|c| !(get_bases(BASE_TYPE::DNA).contains(&c)));
        match invalid {
            Some(x) => Err(x),
            None => Ok(DNA {
                strand: String::from(dna),
            }),
        }
    }

    pub fn to_rna(self) -> RNA {
        let rna = self
            .strand
            .chars()
            .map(|base| BASES.into_iter().find(|b| b.0 == base).unwrap().1)
            .collect::<String>();
        RNA { strand: rna }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let invalid: Option<usize> = rna.chars().position(|c| !(get_bases(BASE_TYPE::RNA).contains(&c)));
        match invalid {
            Some(x) => Err(x),
            None => Ok(RNA {
                strand: String::from(rna),
            }),
        }
    }
}
