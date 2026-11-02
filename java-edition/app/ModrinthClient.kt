package com.oxide.launcher.mods

import org.json.JSONObject
import java.io.File
import java.net.HttpURLConnection
import java.net.URL
import java.net.URLEncoder

/** Finds and downloads Java Edition mods from Modrinth into the profile's mods folder. */
object ModrinthClient {
    private const val API = "https://api.modrinth.com/v2"
    private const val UA = "OxideLauncher/0.1"

    private fun get(url: String): String {
        val c = URL(url).openConnection() as HttpURLConnection
        c.setRequestProperty("User-Agent", UA)
        return c.inputStream.bufferedReader().use { it.readText() }
    }

    fun search(query: String, gameVersion: String, loader: String): List<Pair<String, String>> {
        val facets = URLEncoder.encode("""[["versions:$gameVersion"],["categories:$loader"],["project_type:mod"]]""", "UTF-8")
        val json = JSONObject(get("$API/search?query=${URLEncoder.encode(query, "UTF-8")}&facets=$facets&limit=20"))
        val hits = json.getJSONArray("hits")
        return (0 until hits.length()).map { hits.getJSONObject(it).let { h -> h.getString("slug") to h.getString("title") } }
    }

    /** Downloads the newest compatible file. Returns null if none exists for this version/loader. */
    fun install(slug: String, gameVersion: String, loader: String, modsDir: File): File? {
        val g = URLEncoder.encode("""["$gameVersion"]""", "UTF-8")
        val l = URLEncoder.encode("""["$loader"]""", "UTF-8")
        val versions = org.json.JSONArray(get("$API/project/$slug/version?game_versions=$g&loaders=$l"))
        if (versions.length() == 0) return null
        val file = versions.getJSONObject(0).getJSONArray("files").getJSONObject(0)
        val out = File(modsDir, file.getString("filename"))
        modsDir.mkdirs()
        URL(file.getString("url")).openStream().use { i -> out.outputStream().use { o -> i.copyTo(o) } }
        return out
    }

    /** Disable without deleting: rename .jar <-> .jar.disabled */
    fun setEnabled(jar: File, enabled: Boolean): File {
        val target = if (enabled) File(jar.path.removeSuffix(".disabled")) else File(jar.path + ".disabled")
        if (jar != target) jar.renameTo(target)
        return target
    }
}
