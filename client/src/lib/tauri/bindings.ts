 // This file has been generated by Specta. DO NOT EDIT.

export type CreateUser = { username: string }

export type TauRpcApiInputTypes = { proc_name: "handle_create_user"; input_type: { __taurpc_type: CreateUser } }

export type TauRpcApiOutputTypes = { proc_name: "handle_create_user"; output_type: User }

export type User = { id: number; username: string }

const ARGS_MAP = {"":"{\"handle_create_user\":[\"body\"]}"}
import { createTauRPCProxy as createProxy } from "taurpc"

export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)

type Router = {
	'': [TauRpcApiInputTypes, TauRpcApiOutputTypes],
}