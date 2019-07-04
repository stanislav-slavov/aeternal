import axios from 'axios'

export const actions = {
  getAccountDetails: async function ({ rootState: { nodeUrl }, commit }, account) {
    try {
      const acc = await axios.get(`${nodeUrl}/v2/accounts/${account}`)
      return acc.data
    } catch (e) {
      console.log(e)
      commit('catchError', 'Error', { root: true })
    }
  }
}