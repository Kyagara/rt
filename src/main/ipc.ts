import { BrowserWindow, ipcMain } from 'electron'

import { getUser, listUsers, addUser, removeUser } from './user'
import { getFeed, refreshFeed } from './feed'
import { fetchStream } from './twitch/stream'
import { getUserEmotes } from './twitch/emote'
import { fetchVideo } from './youtube/video'
import { importSubscriptions } from './youtube/channel'

ipcMain.on('titlebar:minimize', (event) => {
	const win = BrowserWindow.fromWebContents(event.sender)
	if (win) {
		win.minimize()
	}
})

ipcMain.on('titlebar:maximize', (event) => {
	const win = BrowserWindow.fromWebContents(event.sender)
	if (win) {
		if (win.isMaximized()) {
			win.unmaximize()
			win.webContents.send('titlebar:event', 'unmaximize')
		} else {
			win.maximize()
			win.webContents.send('titlebar:event', 'maximize')
		}
	}
})

ipcMain.on('titlebar:close', (event) => {
	const win = BrowserWindow.fromWebContents(event.sender)
	if (win) {
		win.close()
	}
})

// User
ipcMain.handle(
	'user:add',
	async (_event, platform, username, id) => await addUser(platform, username, id)
)
ipcMain.handle(
	'user:get',
	async (_event, platform, username, id) => await getUser(platform, username, id)
)
ipcMain.handle('user:list', async (_event, platform) => await listUsers(platform))
ipcMain.handle(
	'user:remove',
	async (_event, platform, username) => await removeUser(platform, username)
)

// Feed
ipcMain.handle('feed:get', async (_event, platform, lastPublishedAt) =>
	getFeed(platform, lastPublishedAt)
)
ipcMain.handle('feed:refresh', async (_event, platform) => await refreshFeed(platform))

// Stream
ipcMain.handle(
	'stream:get',
	async (_event, username, backup) => await fetchStream(username, backup)
)
ipcMain.handle('stream:emotes', async (_event, username) => await getUserEmotes(username))

// Video
ipcMain.handle('video:get', async (_event, videoID) => await fetchVideo(videoID))
ipcMain.handle('video:import', async () => await importSubscriptions())
