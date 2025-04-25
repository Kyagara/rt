import { app, shell, BrowserWindow, ipcMain, Menu } from 'electron'
import path, { join } from 'node:path'
import { electronApp, optimizer, is } from '@electron-toolkit/utils'

import { runMigrations } from './database/migrations'
import { initDatabases } from './database/dbs'
import { handleURL, upsertKeyValue } from './utils'

initDatabases()
runMigrations()
import './ipc'

function createWindow(url?: string): void {
	const window = new BrowserWindow({
		width: 1200,
		height: 600,
		show: false,
		darkTheme: true,
		frame: false,
		webPreferences: {
			preload: join(__dirname, '../preload/index.js')
		}
	})

	window.on('ready-to-show', () => {
		window.show()
	})

	window.webContents.setWindowOpenHandler((details) => {
		shell.openExternal(details.url)
		return { action: 'deny' }
	})

	const { view, path } = handleURL(url)

	if (is.dev && process.env['ELECTRON_RENDERER_URL']) {
		window.loadURL(`${process.env['ELECTRON_RENDERER_URL']}?view=${view}&path=${path}`)
	} else {
		const query: Record<string, string> = { view: view, path: path }
		window.loadFile(join(__dirname, '../renderer/index.html'), { query: query })
	}

	window.on('resized', () => {
		window.webContents.send('titlebar:resized', window.isMaximized())
	})

	window.on('maximize', () => {
		window.webContents.send('titlebar:maximized', true)
	})

	window.on('unmaximize', () => {
		window.webContents.send('titlebar:maximized', false)
	})

	// https://pratikpc.medium.com/bypassing-cors-with-electron-ab7eaf331605
	window.webContents.session.webRequest.onBeforeSendHeaders((details, callback) => {
		const { requestHeaders, referrer } = details
		if (referrer === 'https://www.youtube-nocookie.com/') {
			callback({
				requestHeaders
			})

			return
		}

		upsertKeyValue(requestHeaders, 'Access-Control-Allow-Origin', ['*'])

		callback({ requestHeaders })
	})

	window.webContents.session.webRequest.onHeadersReceived((details, callback) => {
		const { responseHeaders, referrer } = details
		if (referrer === 'https://www.youtube-nocookie.com/') {
			callback({
				responseHeaders
			})

			return
		}

		upsertKeyValue(responseHeaders, 'Access-Control-Allow-Origin', ['*'])
		upsertKeyValue(responseHeaders, 'Access-Control-Allow-Headers', ['*'])

		callback({
			responseHeaders
		})
	})
}

if (process.defaultApp) {
	if (process.argv.length >= 2) {
		app.setAsDefaultProtocolClient('rt', process.execPath, [path.resolve(process.argv[1])])
	}
} else {
	app.setAsDefaultProtocolClient('rt')
}

const gotTheLock = app.requestSingleInstanceLock()
if (!gotTheLock) {
	app.quit()
} else {
	Menu.setApplicationMenu(null)
	app.enableSandbox()
	electronApp.setAppUserModelId('rt')

	app.on('browser-window-created', (_, window) => {
		optimizer.watchWindowShortcuts(window)
	})

	app.on('second-instance', (_event, url) => {
		createWindow(url.join())
	})

	app.on('activate', function () {
		if (BrowserWindow.getAllWindows().length === 0) {
			createWindow()
		}
	})

	app.whenReady().then(() => {
		const url = process.argv.find((arg) => arg.startsWith('rt://'))
		createWindow(url)
	})

	app.on('window-all-closed', () => {
		if (process.platform !== 'darwin') {
			app.quit()
		}
	})
}

ipcMain.on('main:new-window', (_event, url: string) => {
	createWindow(url)
})
