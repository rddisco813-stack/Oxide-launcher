package com.oxide.launcher.account

import android.content.Context
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/**
 * Controls whether a Microsoft account is needed to start the game.
 * Optional ON  -> offline/local accounts can launch (testing, singleplayer).
 * Optional OFF -> a signed-in Microsoft account is required.
 */
object AccountPolicy {
    private const val PREFS = "oxide_account"
    private const val KEY = "microsoft_optional"

    fun isOptional(ctx: Context): Boolean =
        ctx.getSharedPreferences(PREFS, Context.MODE_PRIVATE).getBoolean(KEY, true) // default ON for testing

    fun setOptional(ctx: Context, value: Boolean) {
        ctx.getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit().putBoolean(KEY, value).apply()
    }

    /** Call this right before launching the game. */
    fun canLaunch(ctx: Context, hasMicrosoftAccount: Boolean, hasOfflineAccount: Boolean): Boolean =
        hasMicrosoftAccount || (isOptional(ctx) && hasOfflineAccount)

    /** Offline accounts cannot join online-mode servers or Realms. Show this to the user. */
    const val OFFLINE_NOTE = "Offline accounts work for singleplayer and offline servers only."
}

/** Button/switch for the Accounts screen. */
@Composable
fun MicrosoftOptionalToggle(ctx: Context) {
    var optional by remember { mutableStateOf(AccountPolicy.isOptional(ctx)) }
    Column(Modifier.fillMaxWidth().padding(16.dp)) {
        Row(horizontalArrangement = Arrangement.SpaceBetween, modifier = Modifier.fillMaxWidth()) {
            Text("Make Microsoft login optional", style = MaterialTheme.typography.titleMedium)
            Switch(checked = optional, onCheckedChange = { optional = it; AccountPolicy.setOptional(ctx, it) })
        }
        Text(
            if (optional) "On: you can play with an offline account. ${AccountPolicy.OFFLINE_NOTE}"
            else "Off: a Microsoft account is required to launch.",
            style = MaterialTheme.typography.bodySmall
        )
    }
}
