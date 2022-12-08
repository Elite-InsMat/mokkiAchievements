export interface User {
    username: string,
    achievements: number[],
    timestamps: number[]
}
export interface Achievement {
    name: string,
	description: string,
	image: string,
	id: number,
	unlocked: boolean,
	timestamp: number,
}