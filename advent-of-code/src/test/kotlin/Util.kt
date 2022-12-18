class Util {
    companion object {
        fun getResourceAsText(path: String): String {
            val resource = object {}.javaClass.getResource(path) ?: throw Exception("resource not found: $path")
            return resource.readText()
        }
    }
}
