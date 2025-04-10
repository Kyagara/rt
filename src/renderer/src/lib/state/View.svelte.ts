import { View } from '$shared/enums'

type CurrentView = {
	id: View
	name: string
	route: string
}

// eslint-disable-next-line prefer-const
export let currentView: CurrentView = $state({ id: View.Videos, name: 'Videos', route: '/videos' })

export function changeView(newViewID: View, navigateURL = true, path?: string): void {
	switch (newViewID) {
		case View.Videos:
			localStorage.setItem('lastView', newViewID)

			currentView.id = View.Videos
			currentView.name = 'Videos'
			if (navigateURL) {
				navigate(`/videos${path ? `${path}` : ''}`)
			}
			break

		case View.Streams:
			localStorage.setItem('lastView', newViewID)

			currentView.id = View.Streams
			currentView.name = 'Streams'
			if (navigateURL) {
				navigate(`/streams${path ? `${path}` : ''}`)
			}
			break

		case View.Users:
			localStorage.setItem('lastView', newViewID)

			currentView.id = View.Users
			currentView.name = 'Users'
			if (navigateURL) {
				navigate('/users')
			}
			break
	}
}

function navigate(route: string): void {
	window.history.pushState({}, '', route)
	currentView.route = route
}
