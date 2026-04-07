
// 定义一个缓存trait， 这个trait定义了一个接口， 这个接口有两个方法， get和put， get方法接受一个key， 返回一个Option类型的值， put方法接受一个key和一个value， 没有返回值
pub trait Cache<K, V> {
    fn get(&mut self, key: K) -> Option<V>;
    fn put(&mut self, key: K, value: V);
}