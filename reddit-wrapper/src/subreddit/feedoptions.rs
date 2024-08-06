
pub enum FeedFilter{
    Hot,
    Top,
    New,
    Random,
    Rising,
    Controversial
}
pub enum FeedSort{
    Latest
}

impl FeedSort{
    pub fn as_str(&self)->&str{
        match self{
            FeedSort::Latest=>"latest",
        }

    }

}


impl FeedFilter{
    pub fn as_str(&self)->&str{
        match self{
            FeedFilter::Hot=>"hot",
            FeedFilter::Controversial=>"controversial",
            FeedFilter::Random=>"random",
            FeedFilter::New=>"new",
            FeedFilter::Top=>"top",
            FeedFilter::Rising=>"rising"
        }

    }
}
