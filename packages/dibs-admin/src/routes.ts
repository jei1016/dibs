import { defineRoutes } from "@bearcove/dibs-router";
import DashboardView from "./views/DashboardView.svelte";
import TableListView from "./views/TableListView.svelte";
import RowDetailView from "./views/RowDetailView.svelte";
import RowCreateView from "./views/RowCreateView.svelte";

/**
 * Route definitions for dibs-admin.
 * These are relative - the consuming app mounts them at a prefix (e.g., "/admin/*").
 */
export const adminRoutes = defineRoutes({
  dashboard: {
    path: "/",
    component: DashboardView,
  },
  tableList: {
    path: "/:table",
    query: {
      page: { type: "number", default: 1 },
      sort: { type: "string", optional: true },
      sortDir: { type: "string", optional: true },
    },
    component: TableListView,
  },
  rowCreate: {
    path: "/:table/new",
    component: RowCreateView,
  },
  rowDetail: {
    path: "/:table/:pk",
    component: RowDetailView,
  },
});

export type AdminRoutes = typeof adminRoutes;
