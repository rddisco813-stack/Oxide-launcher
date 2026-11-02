package com.oxide.launcher.mods

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import java.io.File

enum class Edition { JAVA, RUST }

/** Mods tab: toggle mods on/off and add more. Works for both editions. */
@Composable
fun ModsScreen(edition: Edition, mods: List<File>, onToggle: (File, Boolean) -> Unit, onAdd: () -> Unit, onBrowse: () -> Unit) {
    Column(Modifier.fillMaxSize().padding(16.dp)) {
        Text(if (edition == Edition.JAVA) "Java mods" else "Rust mods", style = MaterialTheme.typography.titleLarge)
        Row(Modifier.padding(vertical = 8.dp), horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            Button(onClick = onAdd) { Text("Add from file") }
            if (edition == Edition.JAVA) OutlinedButton(onClick = onBrowse) { Text("Browse Modrinth") }
        }
        LazyColumn {
            items(mods) { f ->
                var on by remember { mutableStateOf(!f.name.endsWith(".disabled")) }
                ListItem(
                    headlineContent = { Text(f.name.removeSuffix(".disabled")) },
                    trailingContent = { Switch(checked = on, onCheckedChange = { on = it; onToggle(f, it) }) }
                )
                HorizontalDivider()
            }
        }
    }
}
