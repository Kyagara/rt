import { contextBridge, ipcRenderer } from 'electron'

import { Platform } from '$shared/enums'

try {
	contextBridge.exposeInMainWorld('main', {
		newWindow: (route: string) => ipcRenderer.send('main:new-window', route)
	})

	contextBridge.exposeInMainWorld('titlebar', {
		onResized: (callback: (isMaximized: boolean) => void) => {
			ipcRenderer.on('titlebar:resized', (_event, isMaximized: boolean) => {
				callback(isMaximized)
			})
		},
		onMaximized: (callback: (isMaximized: boolean) => void) => {
			ipcRenderer.on('titlebar:maximized', (_event, isMaximized: boolean) => {
				callback(isMaximized)
			})
		},
		minimize: () => ipcRenderer.send('titlebar:minimize'),
		maximize: () => ipcRenderer.send('titlebar:maximize'),
		close: () => ipcRenderer.send('titlebar:close')
	})

	contextBridge.exposeInMainWorld('user', {
		add: (platform: Platform, username: string) =>
			ipcRenderer.invoke('user:add', platform, username),
		get: (platform: Platform, username?: string, id?: string) =>
			ipcRenderer.invoke('user:get', platform, username, id),
		list: (platform?: Platform) => ipcRenderer.invoke('user:list', platform),
		remove: (platform: Platform, username: string) =>
			ipcRenderer.invoke('user:remove', platform, username)
	})

	contextBridge.exposeInMainWorld('feed', {
		get: (platform: Platform, lastPublishedAt?: number) =>
			ipcRenderer.invoke('feed:get', platform, lastPublishedAt),
		refresh: (platform: Platform) => ipcRenderer.invoke('feed:refresh', platform)
	})

	contextBridge.exposeInMainWorld('stream', {
		get: (username: string, backup: boolean) => ipcRenderer.invoke('stream:get', username, backup),
		emotes: (username: string) => ipcRenderer.invoke('stream:emotes', username),
		info: (username: string) => ipcRenderer.invoke('stream:info', username)
	})

	contextBridge.exposeInMainWorld('video', {
		get: (videoID: string, retrievePlayer: boolean) =>
			ipcRenderer.invoke('video:get', videoID, retrievePlayer),
		import: () => ipcRenderer.invoke('video:import')
	})
} catch (err) {
	console.error(err)
}
