struct Base();
struct Breed<'a>(&'a str);
struct SubBreed<'a>(&'a str, &'a str);

#[derive(Clone, Copy, Default, Debug)]
pub struct Request<State> {
    state: State,
}

impl<State> Request<State> {
    pub const BASE_URL: &'static str = "https://dog.ceo/api";
}

impl Request<Base> {
    pub fn new() -> Request<Base> {
        Request { state: Base() }
    }
}

impl Request<Base> {
    /// return the url to get a list of all breeds
    pub fn list_breeds_url(&self) -> String {
        format!("{}/breeds/list", Self::BASE_URL)
    }

    /// return the url to get a list of all breeds and sub-breeds
    pub fn list_all_breeds_url(&self) -> String {
        format!("{}/breeds/list/all", Self::BASE_URL)
    }

    /// return the url to get one random dog url
    pub fn random_url_url(&self) -> String {
        format!("{}/breeds/image/random", Self::BASE_URL)
    }

    /// return the url to get n random dog url
    pub fn random_urls_url(&self, n: usize) -> String {
        format!("{}/breeds/image/random/{}", Self::BASE_URL, n)
    }

    /// specify a specific breed and update the state of the request
    pub fn breed<'a>(self, breed: &'a str) -> Request<Breed<'a>> {
        Request {
            state: Breed(breed),
        }
    }
}

impl<'a> Request<Breed<'a>> {
    /// return the url to get all the images url for a specified breed
    pub fn all_images_urls_url(&self) -> String {
        format!("{}/breed/{}/images", Self::BASE_URL, self.state.0)
    }

    /// return the url to get a random image url for a specified breed
    pub fn random_url_url(&self) -> String {
        format!("{}/breed/{}/images/random", Self::BASE_URL, self.state.0)
    }

    /// return the url to get randoms images urls for a specified breed
    pub fn random_urls_url(&self, n: usize) -> String {
        format!(
            "{}/breed/{}/images/random/{}",
            Self::BASE_URL,
            self.state.0,
            n
        )
    }

    /// return the url to list all the sub breed
    pub fn list_all_sub_breeds_url(&self) -> String {
        format!("{}/breed/{}/list", Self::BASE_URL, self.state.0)
    }

    /// specify a specific sub breed and update the state of the request
    pub fn sub_breed(self, sub_breed: &'a str) -> Request<SubBreed<'a>> {
        Request {
            state: SubBreed(self.state.0, sub_breed),
        }
    }
}

impl<'a> Request<SubBreed<'a>> {
    /// return the url to get all the images url for a specified sub breed
    pub fn list_all_images_urls_url(&self) -> String {
        format!(
            "{}/breed/{}/{}/images",
            Self::BASE_URL,
            self.state.0,
            self.state.1
        )
    }

    /// return the url to get a random image url for a specified sub breed
    pub fn random_image_url_url(&self) -> String {
        format!(
            "{}/breed/{}/{}/images/random",
            Self::BASE_URL,
            self.state.0,
            self.state.1
        )
    }

    /// return the url to get randoms images urls for a specified sub breed
    pub fn random_image_urls_url(&self, n: usize) -> String {
        format!(
            "{}/breed/{}/{}/images/random/{}",
            Self::BASE_URL,
            self.state.0,
            self.state.1,
            n
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_breeds_url() {
        assert_eq!(
            Request::new().list_breeds_url(),
            "https://dog.ceo/api/breeds/list"
        );
    }

    #[test]
    fn test_list_all_breeds_url() {
        assert_eq!(
            Request::new().list_all_breeds_url(),
            "https://dog.ceo/api/breeds/list/all"
        );
    }

    #[test]
    fn test_random_url() {
        assert_eq!(
            Request::new().random_url_url(),
            "https://dog.ceo/api/breeds/image/random"
        );
    }

    #[test]
    fn test_random_urls() {
        assert_eq!(
            Request::new().random_urls_url(3),
            "https://dog.ceo/api/breeds/image/random/3"
        );
    }

    #[test]
    fn test_breed_all_images_urls() {
        assert_eq!(
            Request::new().breed("hound").all_images_urls_url(),
            "https://dog.ceo/api/breed/hound/images"
        );
    }

    #[test]
    fn test_breed_random_image() {
        assert_eq!(
            Request::new().breed("hound").random_url_url(),
            "https://dog.ceo/api/breed/hound/images/random"
        );
    }

    #[test]
    fn test_breed_random_images() {
        assert_eq!(
            Request::new().breed("hound").random_urls_url(3),
            "https://dog.ceo/api/breed/hound/images/random/3"
        );
    }

    #[test]
    fn test_list_all_sub_breeds() {
        assert_eq!(
            Request::new().breed("hound").list_all_sub_breeds_url(),
            "https://dog.ceo/api/breed/hound/list"
        );
    }

    #[test]
    fn test_list_all_sub_breeds_images_urls() {
        assert_eq!(
            Request::new()
                .breed("hound")
                .sub_breed("afghan")
                .list_all_images_urls_url(),
            "https://dog.ceo/api/breed/hound/afghan/images"
        );
    }

    #[test]
    fn test_sub_breed_random_image() {
        assert_eq!(
            Request::new()
                .breed("hound")
                .sub_breed("afghan")
                .random_image_url_url(),
            "https://dog.ceo/api/breed/hound/afghan/images/random"
        );
    }

    #[test]
    fn test_sub_breed_random_images() {
        assert_eq!(
            Request::new()
                .breed("hound")
                .sub_breed("afghan")
                .random_image_urls_url(3),
            "https://dog.ceo/api/breed/hound/afghan/images/random/3"
        );
    }
}
