import {Player, system} from "@minecraft/server";

//interfaces
export interface ListdResult {
  activeSessionId: string;
  clientId: string;
  color: string;
  deviceSessionId: string;
  globalMultiplayerCorrelationId: string;
  id: number;
  name: string;
  pfid: string;
  randomId: number;
  "split-screen-player": boolean;
  uuid: string;
  xuid: string;
}
export interface ListdStatsResult extends ListdResult {
  avgpacketloss: number;
  avgping: number;
  maxbps: number;
  packetloss: number;
  ping: number;
}
export interface ListdCustomResult {
  success: boolean;
  player_id: number;
  is_stats: false;
  data?: ListdResult;
}
export interface ListdStatsCustomResult {
  success: boolean;
  player_id: number;
  is_stats: true;
  data?: ListdStatsResult;
}
export type ListdCustomResultType = ListdCustomResult | ListdStatsCustomResult;

export type ListdResultType = ListdResult | ListdStatsResult;

//queue
interface ReqValue {
  resolve: (value: ListdResultType | PromiseLike<ListdResultType>) => void;
  reject: (reason: Error) => void;
  timeoutJob: number;
}

const queue = new Map<string, ReqValue>();

//consts
const _timeoutDuration = 20 * 10;

//utils
function _genReqKey(playerId: string, isStats: boolean): string {
  return `${playerId}-${isStats}`;
}

//main

/**
 * @param target The Player object.
 * @param isStats (Optional) Specifies whether to retrieve statistical data.
 * @returns Returns Promise either a `ListdResult` or `ListdStatsResult` based on the `isStats` value.
 * @remark This function can be called even if Player.isValid is false.
 * @throws Throws if player is invalid, or if the request times out.
 */
export async function listd(target: Player, isStats: true): Promise<ListdStatsResult>;
/**
 * @param target The Player object.
 * @param isStats (Optional) Specifies whether to retrieve statistical data.
 * @returns Returns Promise either a `ListdResult` or `ListdStatsResult` based on the `isStats` value.
 * @remark This function can be called even if Player.isValid is false.
 * @throws Throws if player is invalid, or if the request times out.
 */
export async function listd(target: Player, isStats?: false): Promise<ListdResult>;

export async function listd(target: Player, isStats: boolean = false): Promise<ListdResultType> {
  const playerId = target.id;
  console.warn(`listd:{"isStats":${isStats},"playerId":${playerId}}`);
  return new Promise<ListdResultType>((resolve, reject) => {
    const key = _genReqKey(playerId, isStats);
    const timeoutJob = system.runTimeout(() => {
      const isValid = queue.delete(key);
      if (isValid) {
        reject(
          new Error(
            `Listd request for player ${playerId} timed out after ${_timeoutDuration / 20} seconds.`,
          ),
        );
      }
    }, _timeoutDuration);
    queue.set(key, {resolve: resolve, reject: reject, timeoutJob: timeoutJob});
  });
}

//scriptevent
system.afterEvents.scriptEventReceive.subscribe(ev => {
  if (ev.id === "listd:result") {
    try {
      const result = JSON.parse(ev.message) as ListdCustomResultType;
      const playerId = result.player_id;
      const isStats = result.is_stats;
      const str = _genReqKey(playerId.toString(), isStats);
      const req = queue.get(str);
      if (!result.success) {
        if (req) {
          req.reject(new Error("Request/Player Invalid."));
          queue.delete(str);
        }
      } else if (result.data) {
        if (req) {
          req.resolve(result.data);
          system.clearRun(req.timeoutJob);
          queue.delete(str);
        }
      }
    } catch (e) {
      console.error(`Error processing listd:result for message "${ev.message}":`, e);
    }
  }
});
